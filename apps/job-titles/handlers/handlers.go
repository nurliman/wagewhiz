package handlers

import (
	"context"
	"net/http"
	"strconv"
	"time"

	influxdb2 "github.com/influxdata/influxdb-client-go/v2"
	"github.com/labstack/echo/v4"
	"github.com/meilisearch/meilisearch-go"
	"wagewhiz.nurliman.dev/action-catcher/conn"
)

const DefaultJobTitlesLimit = 5
const MaxJobTitlesLimit = 20

func SeachJobTitles(c echo.Context) error {
	var query string = c.QueryParam("q")
	var limitStr string = c.QueryParam("limit")
	var limit int = DefaultJobTitlesLimit

	if limitStr != "" {
		parsedLimit, err := strconv.Atoi(limitStr)

		if err != nil {
			return c.JSON(http.StatusBadRequest, map[string]string{
				"message": "Invalid limit value",
			})
		}

		limit = parsedLimit
	}

	// Calculate the effective limit based on input and default values
	effLimit := DefaultJobTitlesLimit
	if limit > 0 && limit <= MaxJobTitlesLimit {
		effLimit = limit
	} else if limit > MaxJobTitlesLimit {
		effLimit = MaxJobTitlesLimit
	}

	meilisearchClient := conn.GetMeiliSearchClient()
	searchRes, err := meilisearchClient.Index("job-titles").Search(query,
		&meilisearch.SearchRequest{
			Limit: int64(effLimit),
		})

	if err != nil {
		c.Logger().Error(err)
		return c.JSON(http.StatusInternalServerError, map[string]string{
			"message": "Something went wrong",
		})
	}

	return c.JSON(http.StatusOK, searchRes.Hits)
}

func UserPickJobTitle(c echo.Context) error {
	jobTitleIdStr := c.Param("id")

	if jobTitleIdStr == "" {
		return c.JSON(http.StatusBadRequest, map[string]string{
			"message": "Job title id is required",
		})
	}

	go func(jobTitleId string) {
		influxdbClient := conn.GetInfluxDBClient()
		writeAPI := influxdbClient.WriteAPIBlocking("wagewhiz", "wagewhiz")

		p := influxdb2.NewPoint("user_pick_job_title",
			map[string]string{},
			map[string]interface{}{
				"job_title_id": jobTitleId,
			},
			time.Now(),
		)

		err := writeAPI.WritePoint(context.Background(), p)

		if err != nil {
			c.Logger().Error(err)
		}
	}(jobTitleIdStr)

	return c.JSON(http.StatusOK, map[string]string{
		"message": "OK",
	})
}

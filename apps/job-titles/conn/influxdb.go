package conn

import (
	"context"
	"log"
	"sync"

	influxdb2 "github.com/influxdata/influxdb-client-go/v2"
)

var influxdbOnce sync.Once
var influxdbClient influxdb2.Client

func GetInfluxDBClient() influxdb2.Client {
	influxdbOnce.Do(func() {
		influxdbClient = influxdb2.NewClient("http://localhost:8086", "wYPfbozT3vedejmiVf3COe_PUX2Ni2TQwkAnWJoMrd296EfY2luYj0SZEp-JLUF0Wmjw5wcsPMEKruFbO81Rpg==")

		_, err := influxdbClient.Health(context.Background())
		if err != nil {
			log.Fatal(err)
		}
	})

	return influxdbClient
}

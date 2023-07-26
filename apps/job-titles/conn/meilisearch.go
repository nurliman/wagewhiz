package conn

import (
	"log"
	"sync"

	"github.com/meilisearch/meilisearch-go"
)

var meilisearchOnce sync.Once
var meilisearchClient *meilisearch.Client

func GetMeiliSearchClient() *meilisearch.Client {
	meilisearchOnce.Do(func() {
		client := meilisearch.NewClient(meilisearch.ClientConfig{
			Host:   "http://127.0.0.1:7700",
			APIKey: "MASTER_KEY",
		})

		_, err := client.Health()
		if err != nil {
			log.Fatal(err)
		}

		meilisearchClient = client
	})

	return meilisearchClient
}

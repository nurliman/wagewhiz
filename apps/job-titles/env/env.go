package env

import (
	"log"
	"sync"

	"github.com/caarlos0/env/v9"
	"github.com/joho/godotenv"
)

type environtmentVariables struct {
	Host           string `env:"HOST" envDefault:"localhost"`
	Port           int    `env:"PORT" envDefault:"3002"`
	InfluxDBURL    string `env:"INFLUXDB_URL,notEmpty"`
	InfluxDBToken  string `env:"INFLUXDB_TOKEN,notEmpty"`
	InfluxDBOrg    string `env:"INFLUXDB_ORG" envDefault:"wagewhiz"`
	InfluxDBBucket string `env:"INFLUXDB_BUCKET" envDefault:"wagewhiz"`
	MeiliSearchURL string `env:"MEILISEARCH_URL,notEmpty"`
	MeiliSearchKey string `env:"MEILISEARCH_KEY,required"`
}

var envOnce sync.Once
var envVar *environtmentVariables

func GetEnv() *environtmentVariables {
	envOnce.Do(func() {
		err := godotenv.Load()
		if err != nil {
			log.Println("No .env file found")
		} else {
			log.Println("Loaded .env file")
		}

		envVar = &environtmentVariables{}
		if err := env.Parse(envVar); err != nil {
			log.Fatal(err)
		}

	})

	return envVar
}

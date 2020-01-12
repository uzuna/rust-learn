DATABASE_URL:="postgresql://postgres:password@localhost:5432/log-collector"

diesel_setup:
	DATABASE_URL=${DATABASE_URL} diesel setup
	DATABASE_URL=${DATABASE_URL} diesel migration generate logs

diesel_run:
	DATABASE_URL=${DATABASE_URL} diesel migration run
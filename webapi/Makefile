DATABASE_URL:="postgresql://postgres:password@localhost:5432/log-collector"
setup:
	docker-compose up -d

setup_diesel:
	DATABASE_URL=${DATABASE_URL} diesel setup
	DATABASE_URL=${DATABASE_URL} diesel migration generate create_posts

diesel_run:
	DATABASE_URL=${DATABASE_URL} diesel migration run
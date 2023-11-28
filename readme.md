dependencies: cargo

sudo make install
sudo make create_docker_container
sudo make start_docker_db
sudo make create_postgres_db
sudo make migrate-up
sudo make run
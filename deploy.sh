sudo docker-compose down

# Bypass cache and rebuild?
sudo docker-compose rm -f
sudo docker-compose up -d --build
docker stop fastlink-app
docker rm fastlink-app
docker build -t fastlink .
docker run --name fastlink-app \
           --env-file ./.env \
           -p 5000:5000 \
           --volume db:/app/db \
           -d fastlink

docker pull mariadb;
ipaddr=$(hostname -I | grep -oE " \b([0-9]{1,3}\.){3}[0-9]{1,3}\b")
docker run -p $ipaddr:3306:3306 --name some-mariadb -e MYSQL_ROOT_PASSWORD=root -d mariadb
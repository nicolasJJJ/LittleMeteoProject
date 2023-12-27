# goal
The main goal is to publish some recreatives projects for me to learn, exchange with people more professional than me and maybe offer me new opportunities for data engineering projects.

The second goal is to sh*t the mouth of an SIGINT officer with a better skills than me with his "Bambee" sweater. WTf, a Bambee sweater !! It's time to show who's on the top of food chain.


# Little Meteo Project
A Kafka & Spark project with K8s for a meteo app with free open source data.

K8S : 
![DALL·E 2023-11-18 16 49 52 - A muscular wrestler resembling Stone Cold Steve Austin, with a visible K8S tattoo, lifting two massive motorcycle engines  Each engine is emblazoned w](https://github.com/nicolasJJJ/5ShotsOfKafkaInK8sBar/assets/104780543/aa50ace3-3372-49eb-91ad-4d53faaf0d1a)


## Configs folder
Contains configs read by microK8S to start the services.

microk8s.kubectl apply -f zookeeper-deployment.yaml
microk8s.kubectl apply -f kafka-deployment.yaml
microk8s.kubectl apply -f zookeeper-service.yaml
microk8s.kubectl apply -f kafka-service.yaml

# goal
The main goal is to publish some recreatives projects for me to learn, exchange with people more professional than me and maybe offer me new opportunities for data engineering projects.

The second goal is to sh*t the mouth of an SIGINT officer with better skills than me with his "Bambee" sweater. WTf, a Bambee sweater !! It's time to show who's on the top of food chain.


# 5ShotsOfKafkaInK8sBar
A Kafka project with K8s. 

K8S : 
<img src="https://github.com/nicolasJJJ/5ShotsOfKafkaInK8sBar/assets/104780543/aa50ace3-3372-49eb-91ad-4d53faaf0d1a" width="50%">


## stunner_brew_admiral

The cargo directory containing the kafka producer

## Configs folder
Contains configs read by microK8S to start the services.

```
microk8s.kubectl apply -f zookeeper-deployment.yaml
microk8s.kubectl apply -f kafka-deployment.yaml
microk8s.kubectl apply -f zookeeper-service.yaml
microk8s.kubectl apply -f kafka-service.yaml
```

maybe i'll start with a bash file that will make every steps.
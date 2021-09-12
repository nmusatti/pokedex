### How to deploy Pokedex to Kubernetes

The YAML manifests in this directory may be used as they are to deploy Pokedex to a Minikube cluster
or as a starting point for custom deployments.

On a Linux system you may install Minikube by running the following commands:

```bash
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube
```

Then you can start it with:

```bash
minikube start
```

In a production setting usually container images are kept in registries and Kubernetes would pull
Pokedex's image from one of those. To avoid this need you can rebuild the Pokedex project within
Minikube's Docker context by running the following commands:

```bash
eval $(minikube -p minikube docker-env)
docker build -t pokedex .
```

To deploy Pokedex to Minikube you may run the following commands:

```bash
minikube kubectl -- apply -f namespace.yaml
minikube kubectl -- apply -f deployment.yaml
minikube kubectl -- apply -f service.yaml
```

To make Pokedex accessible to your browser you need to run the following command:

```bash
minikube kubectl -- port-forward service/pokedex 7000:8000 -n pokedex
```

Now you can point your browser to (note the different port number):

```
http://localhost:7000/pokemon/translated/butterfree
```

# Rust Kubernetes Hello World Server

The following has been tested with kubernetes disribution k3s running on WSL2 Windows.

## Build and Publish the Image
You can skip this if you don't want to use your own image. If you use your own image, replace the
image used inside "hello-world-kubernetes-manifest.yaml"

Dockerization steps:
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

docker build . -t hello-world-kubernetes
```

Test dockerization worked:
```bash
docker run -d -p 8000:8000 hello-world-kubernetes

curl localhost:8000
```
Shutdown container:
```bash
# Get running container name
docker ps 
docker stop <Container running name>
```
Publish to dockerhub
```bash
docker tag hello-world-kubernetes:latest <YOUR_DOCKERHUB_NAMESPACE>/hello-world-kubernetes:latest
docker push <YOUR_DOCKERHUB_NAMESPACE>/hello-world-kubernetes:latest
```

## Kubernetes
Install k3s:
https://www.guide2wsl.com/k3s/

Open WSL2 distro and navigate here. Then:
```bash
kubectl apply -f hello-world-kubernetes-manifest.yaml
```

Check Resources were created:
```bash
kubectl get all
```
```bash
curl localhost:80
```
Done.
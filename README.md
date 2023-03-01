# Kubernetes-Based-Continous-Delivery-of-Actix-Microservice

This is a simple Actix microservice with five routes that return different responses to incoming requests. The microservice is built using the Actix-Web framework and uses Rust programming language.

# Routes
The microservice has the following routes:

/ : returns a string "Hello, World!"
/health : returns a 200 status code
/Gold : returns a string "Gold Status"
/Silver : returns a string "Silver Status"
/Bronze : returns a string "Bronze Status"

# Usage
To use this microservice, follow these steps:

Clone the repository on your local machine using the git clone command.
Make sure you have Rust programming language installed on your machine.
Navigate to the project directory using your terminal.
Run the command cargo run to start the server.
Visit http://127.0.0.1:8080/ to see the "Hello, World!" message.
Visit http://127.0.0.1:8080/health to get a 200 status code.
Visit http://127.0.0.1:8080/Gold to get "Gold Status" message.
Visit http://127.0.0.1:8080/Silver to get "Silver Status" message.
Visit http://127.0.0.1:8080/Bronze to get "Bronze Status" message.

## Configuration Steps

1. Create a Virtual Environment

The purpose of virtual environments is to create a self-contained environment for each of your projects, allowing you to manage dependencies, libraries, and versions separately for each project.

    `python3 -m venv rustenv`

    `source rustenv/bin/activate`

    `cd rustenv`

2. Install Rust
Go to https://rustup.rs/ and run the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

    Run `source "$HOME/.cargo/env"` to configure your current shell.

3. create new project
The cargo tool is the default package manager for Rust and provides an easy way to manage dependencies and build projects.

    Run `cargo new` (project name) (my Eg: `cargo new hello`)

This will create a binary (application) `microservice` package

4. Create `main.rs` and `lib.rs` files in the src project

    `touch main.rs` and `touch lib.rs` 

5. Run `Cargo build`   
This is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The cargo build command can be run from the root directory of the project.

5. Set up Cargo.toml to determine the dependencies and build configuration of the project.


## Kubernetes Deployment 
# Minikube Lab (based on official tutorial on https://kubernetes.io)

Launch GitHub Codespace
Run `minikube start` to start cluster
Run `minikube dashboard --url` to view dashboard in a new terminal
Hover over link and "follow link"
Create a deployment: `kubectl create deployment hello-node --image=registry.k8s.io/e2e-test-images/agnhost:2.39 -- /agnhost netexec --http-port=8080`
View deployment: `kubectl get deployments`
View pods: `kubectl get pods`
Create service and expose it: `kubectl expose deployment hello-node --type=LoadBalancer --port=8080`
View services: `kubectl get services`
Curl the url shown, for example: `curl http://192.168.49.2:31839` or change to your URL.

## Cleanup

`kubectl delete service hello-node`
`kubectl delete deployment hello-node`
`minikube stop`

# Deploy with Kubernetes 
Push container to DockerHub (Optional): i.e. `docker build -t <hub-user>/<repo-name>[:<tag>] .` and `docker push <hub-user>/<repo-name>:<tag> .`

`minikube start`
`minikube dashboard --url`
Hover over link and "follow link"
Create a deployment: `kubectl create deployment hello-fastapi --image=registry.hub.docker.com/noahgift/fastapi-kube`
View deployment: `kubectl get deployments`
Create service and expose it: `kubectl expose deployment hello-fastapi --type=LoadBalancer --port=8080`
View services: `kubectl get service actix-service `
`minikube service actix-service --url`
Curl web service: i.e. `curl http://192.168.49.2:31224`

## Cleanup
Cleanup

`kubectl delete service hello-fastapi`
`kubectl delete deployment hello-fastapi`
`minikube stop`



## Conclusion
This is a simple Actix microservice with multiple routes that you can use as a starting point for building your own microservice.

[![Build binary release](https://github.com/nigelmalaba1/Kubernetes-Based-Continous-Delivery-of-Rust-Microservice/actions/workflows/rust.yml/badge.svg)](https://github.com/nigelmalaba1/Kubernetes-Based-Continous-Delivery-of-Rust-Microservice/actions/workflows/rust.yml)

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

1. Clone the repository on your local machine using the git clone command.

2. Make sure you have Rust programming language installed on your machine.

3. Navigate to the project directory using your terminal.

4. Run the command cargo run to start the server.

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


# Kubernetes Deployment 
## Minikube Lab (based on official tutorial on https://kubernetes.io)

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

Create a deployment: `kubectl create deployment hello-api --image=registry.hub.docker.com/<hub-user>/<repo-name>

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

# Architecture Diagram

![Microservices Kubernetes Deployment](https://user-images.githubusercontent.com/123284219/222927001-04bbaac4-f7a3-489a-bb81-d0f4e5b0a236.jpeg)


##Explanation of the architecture:

The microservices are developed using Rust programming language on AWS Cloud9, which is a cloud-based integrated development environment (IDE). The code is version-controlled using GitHub and is stored in a GitHub repository.

The developers use GitHub Codespaces, a cloud-based development environment that allows developers to develop and test code without having to set up a local development environment.

The Docker images for the microservices are built and stored in a Docker registry, such as Docker Hub.

The Kubernetes cluster is created using Minikube, a tool that allows you to run a Kubernetes cluster on a single machine. The deployment object is defined in Kubernetes to deploy the microservices and manage their resources.

The service exposes the microservices to the outside world and routes the requests to the appropriate Kubernetes pods.

Overall, this architecture enables developers to work on the microservices in a cloud-based environment without having to worry about local setup. The microservices can be easily deployed and scaled using Kubernetes.

## Project Presentation

https://www.beautiful.ai/player/-NPQacppTuo7JwNYD92O

## Conclusion
This is a simple Actix microservice with multiple routes that you can use as a starting point for building your own microservice.

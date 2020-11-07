# k8s-rust-test

This is a very simple rust based web application with two endpoints to test a kubernetes cluster.
It can be deployed to Kubernetes using the two yaml files for the deployment and the service.

Calling the / endpoint will show if the application is still healthy.
Calling /maybePanic will kill the application in 50% of the cases.

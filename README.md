# k8s-rust-test

This is a very simple rust based web application with two endpoints to test a kubernetes cluster. It can be deployed to Kubernetes using the two yaml files for the deployment and the service as follows:

`kubectl apply -f manifest.yaml`

`kubectl apply -f service-manifest.yaml`

Once the pods are running you can access the test service on your Nodes on Port `30024`. There are two endpoints:

`/` will show if the application is still healthy
`/maybePanic` will kill the application in 50% of the cases



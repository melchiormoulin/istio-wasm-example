#noobi getmesh istioctl install --set profile=demo

kubernetes way 
getmesh istioctl operator init
kubectl apply -f istio-operator.yaml
kubectl create namespace bookinfo
kubectl label namespace bookinfo istio-injection=enabled --overwrite
#OPT to deploy an app: go to the istio github repo and kubectl apply -n bookinfo -f samples/bookinfo/platform/kube/bookinfo.yaml


wasme way
wasme init ./ --language rust --platform istio --platform-version 1.9.x
#Modify code filter.rs
#rm not needed depencies in BUILD
wasme build rust ./ -t webassemblyhub.io/melki/hello_world:v0.10
wasme push webassemblyhub.io/melki/hello_world:v0.10 --username myuser --password mypass
wasme deploy istio webassemblyhub.io/melki/hello_world:v0.10 \
    --id=hello-world \
    --namespace bookinfo \
    --root-id=root_id


CODE FROM : https://github.com/proxy-wasm/proxy-wasm-rust-sdk/blob/master/examples/http_headers.rs
CONFIG FROM : https://github.com/istio-ecosystem/wasm-extensions/blob/master/example/config/example-filter.yaml

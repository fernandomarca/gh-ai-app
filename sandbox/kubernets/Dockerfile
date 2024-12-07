FROM ubuntu:22.04

RUN apt-get update && apt-get install -y apt-transport-https ca-certificates curl gpg

RUN curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.31/deb/Release.key | gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg

RUN echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.31/deb/ /' | tee /etc/apt/sources.list.d/kubernetes.list   

RUN apt-get update && apt-get install -y kubelet kubeadm kubectl
RUN apt-mark hold kubelet kubeadm kubectl
RUN systemctl enable --now kubelet
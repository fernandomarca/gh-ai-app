#!/bin/bash

# Desabilitar swap
swapoff -a

# Iniciar containerd
containerd &

# Esperar o containerd iniciar
sleep 5

# Iniciar kubelet com a flag --fail-swap-on=false
kubelet --fail-swap-on=false
#!/bin/bash
echo "Enter the Postgres password, even if you don't see a prompt"
kubectl run -i --tty busybox --image=governmentpaas/psql -- sh -c 'psql -h postgres-beta -U postgres'
kubectl delete pod busybox

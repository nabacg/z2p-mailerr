#!/usr/bin/env bash


doctl apps update   $(doctl apps list | grep z2p | cut -d ' ' -f 1) --spec=../spec.yaml

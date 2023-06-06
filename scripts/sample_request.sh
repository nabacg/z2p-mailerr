#!/usr/bin/env bash

curl -v http://127.0.0.1:8080/subscriptions -X POST -H "Content-Type: application/x-www-form-urlencoded" -d "name=le%20guin&email=ursula_le_guin%40gmail.com"
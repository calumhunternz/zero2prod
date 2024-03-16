#!/usr/bin/env bash

eval $(ssh-agent -s)
ssh-add ~/.ssh/github
# NEED TO RUN COMMANDS IN TERMINAL THEY DONT SAVE WHEN RUNNIGN FROM SCRIPT
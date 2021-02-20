FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y -q curl git sudo wget python2 build-essential

RUN echo "%sudo ALL=(ALL:ALL) NOPASSWD:ALL" >> /etc/sudoers

ARG USER_UID
RUN useradd --create-home --uid ${USER_UID:-1000} --shell /bin/bash user
RUN usermod -aG sudo user

USER user

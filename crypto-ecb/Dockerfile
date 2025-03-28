FROM ubuntu:20.04

RUN apt-get update
RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev python3 python3-pip
RUN pip3 install pycryptodome

ARG USER
ENV USER $USER

WORKDIR /
COPY start.sh /start.sh
RUN chmod 755 /start.sh

RUN useradd -m $USER

EXPOSE 9000

COPY src/. ./src/.
RUN chmod +x ./src/main.py

CMD ["/start.sh"]

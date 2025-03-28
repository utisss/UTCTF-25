FROM ubuntu:20.04

RUN chmod 000 /usr/local/sbin/unminimize

RUN apt-get update
RUN ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime

RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev openssh-server xxd vim git grep coreutils libcap2-bin openssl acl
ARG FLAG
ARG USER1
ARG PASSWORD1
ARG SECRET_USER
ARG SECRET_PASSWORD
ENV USER1 $USER1
ENV PASSWORD1 $PASSWORD1
ENV SECRET_USER $SECRET_USER
ENV SECRET_PASSWORD $SECRET_PASSWORD
ENV FLAG $FLAG

WORKDIR /

RUN PASSWORD_HASH=$(openssl passwd -crypt ${PASSWORD1}) \
    && useradd -m ${USER1} -p ${PASSWORD_HASH}
RUN PASSWORD_HASH2=$(openssl passwd -crypt ${SECRET_PASSWORD}) \
    && useradd -m ${SECRET_USER} -p ${PASSWORD_HASH2} -c ${SECRET_PASSWORD}

RUN echo "$FLAG" > /home/$USER1/flag.txt
RUN chmod 000 /home/$USER1/flag.txt
RUN chmod a=rx /home/$USER1
RUN setfacl -m u:$SECRET_USER:r /home/$USER1/flag.txt
RUN usermod -s /bin/bash $USER1

RUN mkdir -p /run/sshd
COPY sshd_config /etc/sshd_config

COPY start.sh /start.sh
RUN chmod 755 /start.sh
RUN service ssh start
EXPOSE 22
RUN export SECRET_USER=$SECRET_USER
RUN export USER1=$USER1

RUN chmod -R a-w / || true

CMD ["/start.sh"]

FROM ubuntu:20.04

RUN chmod 000 /usr/local/sbin/unminimize

RUN apt-get update
RUN ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime

RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev openssh-server xxd vim git grep coreutils libcap2-bin openssl acl
ARG FLAG
ARG USER1
ARG PASSWORD1
ARG USER2
ENV USER1 $USER1
ENV PASSWORD1 $PASSWORD1
ENV USER2 $USER2
ENV FLAG $FLAG

WORKDIR /

RUN PASSWORD_HASH=$(openssl passwd -crypt ${PASSWORD1}) \
    && useradd -m ${USER1} -p ${PASSWORD_HASH}
RUN useradd -m ${USER2}

RUN echo "$FLAG" > /home/$USER1/flag.txt
RUN chown $USER2:$USER2 /home/$USER1/flag.txt
RUN chmod 700 /home/$USER1/flag.txt
RUN chmod a=rx /home/$USER1
RUN chown $USER2:$USER2 /usr/bin/xxd
RUN chmod u+s /usr/bin/xxd
RUN usermod -s /bin/bash $USER1

RUN mkdir -p /run/sshd
COPY sshd_config /etc/sshd_config

RUN service ssh start
EXPOSE 22

RUN chmod -R a-w / || true

CMD ["/usr/sbin/sshd", "-D"]

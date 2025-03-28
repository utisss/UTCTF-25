FROM ubuntu:16.04

RUN apt-get update
RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev file

ARG USER
ARG FLAG
ENV USER $USER
ENV FLAG $FLAG

WORKDIR /

COPY ./main.c /main.c
COPY Makefile /Makefile
RUN make
RUN rm -f main.c Makefile

RUN echo $FLAG > /flag.txt
RUN chmod a=r /flag.txt

COPY start.sh /start.sh
RUN chmod 755 /start.sh

RUN useradd -m $USER

EXPOSE 9000

CMD ["/start.sh"]


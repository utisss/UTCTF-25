FROM php:7.2.6-apache
RUN docker-php-ext-install mysqli
COPY php.ini ${PHP_INI_DIR}/conf.d/php.ini

EXPOSE 80
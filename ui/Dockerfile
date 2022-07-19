FROM alpine:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

ENV TZ=Etc/UTC

RUN apk update && \
    apk add nginx && \
    adduser -D -g 'www' www && \
    mkdir -p /var/www/html/health && \
    chown -R www:www /var/lib/nginx && \
    chown -R www:www /var/www/html/health

RUN sed -i 's/error_log \/var\/log\/nginx\/error.log warn;/error_log \/dev\/stderr debug;/' /etc/nginx/nginx.conf
RUN sed -i 's/access_log \/var\/log\/nginx\/access.log warn;/access_log \/dev\/stdout main;/' /etc/nginx/nginx.conf

COPY ./public/www/html/health/* /var/www/html/health/
COPY ./nginx/health.conf /etc/nginx/http.d/health.conf

COPY ./nginx/nginx-selfsigned.crt /etc/nginx/site.crt
COPY ./nginx/nginx-selfsigned.key /etc/nginx/site.key
COPY ./nginx/dhparam.pem /etc/nginx/dhparam.pem

RUN chmod 400 /etc/nginx/site.crt && \
    chmod 400 /etc/nginx/site.key

EXPOSE 80/udp
EXPOSE 80/tcp

EXPOSE 443/udp
EXPOSE 443/tcp

CMD [ "nginx", "-g", "daemon off;" ]
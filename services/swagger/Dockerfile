FROM swaggerapi/swagger-ui:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

COPY ./domains.json /usr/share/nginx/html/definitions/domains.json
COPY ./users.json /usr/share/nginx/html/definitions/users.json
COPY ./roles.json /usr/share/nginx/html/definitions/roles.json

EXPOSE 80/tcp
EXPOSE 80/udp

ENV SWAGGER_JSON_URL=/definitions/domains.json
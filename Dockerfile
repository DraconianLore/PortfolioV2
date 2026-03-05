FROM nginx:1.25-alpine

# Copy custom nginx configuration
COPY config/nginx.conf /etc/nginx/nginx.conf

# Copy the pre-built Dioxus bundle into the nginx web root
COPY dist/public /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]

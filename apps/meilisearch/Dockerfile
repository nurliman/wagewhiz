# Use the official Meilisearch image as the base image
FROM getmeili/meilisearch:v1.2

# create a directory to store the dump file
RUN mkdir dumps

# Copy the dump file to the container's /meili_data/dumps directory
COPY ./dumps/* /dumps/

# Set the command to run Meilisearch and import the dump
CMD ["meilisearch", "--import-dump", "/dumps/job-titles.dump"]
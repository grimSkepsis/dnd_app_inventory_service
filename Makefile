# Use dotenv to export variables from .env file
ifneq (,$(wildcard .env))
	include .env
	export $(shell sed 's/=.*//' .env)
endif


init_local_db:
	mkdir -p ../neo4j/data
	mkdir -p ../neo4j/logs
	mkdir -p ../neo4j/import
	mkdir -p ../neo4j/plugins
	pip install python-dotenv
	docker pull neo4j:latest
	docker run --name neo4j -d -p 7474:7474 -p 7687:7687 -v $(shell pwd)/../neo4j/data:/data -v $(shell pwd)/../neo4j/logs:/logs -v $(shell pwd)/../neo4j/import:/var/lib/neo4j/import -v $(shell pwd)/../neo4j/plugins:/plugins --env NEO4J_AUTH=neo4j/$(NEO4J_PASSWORD) neo4j:latest
	@echo "db running on http://localhost:7474/"
run:
	cargo run

# Example target to print Neo4j connection details
print-env:
	@echo "NEO4J_USER: $(NEO4J_USER)"
	@echo "NEO4J_PASSWORD: $(NEO4J_PASSWORD)"

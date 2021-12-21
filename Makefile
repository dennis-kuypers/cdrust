.PHONY: haproxy-agent-deb

all: haproxy-agent-deb

haproxy-agent-deb:
	cargo b --release -p haproxy-agent && cargo deb -p haproxy-agent --no-build
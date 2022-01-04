.PHONY: haproxy-agent

haproxy-agent:
	cargo b --release -p haproxy-agent
	cargo deb -p haproxy-agent --no-build
default:
	@echo "No target specified"
	@exit 1

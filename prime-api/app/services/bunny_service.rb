require 'bunny'
require 'json'

class BunnyService
    def initialize
        @connection = Bunny.new(hostname: 'rabbit')
        @connection.start
        @channel = @connection.create_channel
        @task_queue = @channel.queue('task')
        @update_queue = @channel.queue('update')
    end

    def push_task(json)
        @channel.default_exchange.publish(JSON.generate(json), routing_key: @task_queue.name)
    end

    def get_updates
        @update_queue.subscribe(block: true) do |_delivery_info, _properties, body|
            yield(body)
        end
    end
end
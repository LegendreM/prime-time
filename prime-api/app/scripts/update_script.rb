require 'json'

def bunny_loop(bun)
    bun.get_updates do |body|
        begin
            update = JSON.parse(body)
            task = Task.find(update["token"])
            if update["state"] == "done"
                task.update(state: update["state"], result: update["result"])
            else
                task.update(state: update["state"])
            end
        rescue Exception => e
            p e.message
        end
    end
end

p "run"
while 1
    begin
        bun = BunnyService::new
        bunny_loop(bun)
    rescue Exception => e
        p e.message
        sleep 3
    end
end
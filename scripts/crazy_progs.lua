local Direction = require("scripts/libs/direction")

local area_id = "default"

local bot_pos = Net.get_object_by_name(area_id, "Random Bot")

local bot_array = {}

local mug_texture_path = "resources/ow/prog/prog_mug.png"
local mug_animation_path = "resources/ow/prog/prog_mug.animation"

local last_delta_acc = 0

-- Net:on("tick", function(event)
--     -- { delta_time: number (seconds) }
--     last_delta_acc = last_delta_acc + event.delta_time * 1
--     if last_delta_acc < 2 then
--         return
--     end
--     last_delta_acc = 0
--     print("Spawning bot")
--     -- Randomize the bot position a bit
--     local this_bot_pos = {
--         x = bot_pos.x + math.random(-1, 1),
--         y = bot_pos.y + math.random(-1, 1),
--         z = bot_pos.z
--     }
--     local bot_name = "Random Bot " .. math.random(1, 10)
--     local random_bot_id = Net.create_bot({
--       name = bot_name,
--       area_id = area_id,
--       texture_path = "/server/assets/community/prog.png",
--       animation_path = "/server/assets/community/prog.animation",
--       x = this_bot_pos.x,
--       y = this_bot_pos.y,
--       z = this_bot_pos.z,
--       solid = true
--     })
--     table.insert(bot_array, random_bot_id)
--     if #bot_array > 9 then
--         local deleting_bot_id = bot_array[math.random(1, #bot_array)]
--         Net.remove_bot(deleting_bot_id, true)
--         for i, bot_id in ipairs(bot_array) do
--             if bot_id == deleting_bot_id then
--                 table.remove(bot_array, i)
--                 break
--             end
--         end
--     end
-- end)

-- Net:on("player_move", function(event)
--     -- Net.get_player_position(event.player_id) will report the old position
--     -- { player_id: string, x: number, y: number, z: number }
--     local player_pos = {
--         x = event.x,
--         y = event.y,
--         z = event.z
--     }
--     -- If the distance between the player and the bot is greater than 5, move the bot towards the player
--     for _, bot_id in ipairs(bot_array) do
--         local bot_pos = Net.get_bot_position(bot_id)
--         local player_pos = Net.get_player_position(event.player_id)
--         local distance = math.sqrt((bot_pos.x - player_pos.x)^2 + (bot_pos.y - player_pos.y)^2)
--         local follow_distance = tonumber(string.match(Net.get_bot_name(bot_id), "%d+")) * 0.1
--         if distance > follow_distance then
--             local move_distance = 0.2
--             -- determine the position to move to, which is 0.2 closer to the player, in xy plane
--             local move_x = bot_pos.x + (player_pos.x - bot_pos.x) / distance * move_distance
--             local move_y = bot_pos.y + (player_pos.y - bot_pos.y) / distance * move_distance
--             Net.move_bot(bot_id, move_x, move_y, bot_pos.z)
--         end
--     end
    
-- end)

local Direction = require("scripts/libs/direction")

local area_id = "default"

local bot_pos = Net.get_object_by_name(area_id, "Welcome Bot")

local bot_id = Net.create_bot({
  name = "Welcome Bot",
  area_id = area_id,
  texture_path = "/server/assets/community/prog.png",
  animation_path = "/server/assets/community/prog.animation",
  x = bot_pos.x,
  y = bot_pos.y,
  z = bot_pos.z,
  solid = true
})

local mug_texture_path = "resources/ow/prog/prog_mug.png"
local mug_animation_path = "resources/ow/prog/prog_mug.animation"

Net.set_bot_direction(bot_id, Direction.DOWN_LEFT)

Net:on("actor_interaction", function(event)
  if event.actor_id ~= bot_id then
      return
  end

  local player_id = event.player_id

  Net.lock_player_input(player_id)

  Net.set_bot_direction(bot_id, Direction.from_points(bot_pos, Net.get_player_position(player_id)))

  local temperature = Net.get_system_temperature()
  temperature = string.format("%.2f", temperature)

  Async.message_player(player_id, "HELLO! WELCOME TO TEQ'S WEBPAGE!", mug_texture_path, mug_animation_path).and_then(function()
    Async.message_player(player_id, "THE CPU TEMPERATURE IS " .. temperature .. " CELSIUS!", mug_texture_path, mug_animation_path).and_then(function()
      Net.unlock_player_input(player_id)
      Net.set_bot_direction(bot_id, Direction.DOWN_LEFT)
    end)
  end)

  -- Async.question_player(player_id, "HELLO! ARE YOU DOING WELL TODAY?", mug_texture_path, mug_animation_path)
  --   .and_then(function(response)
  --     if response == nil then
  --       -- player disconnected
  --       return
  --     end

  --     if response == 1 then
  --       Net.message_player(player_id, "THAT'S GREAT!", mug_texture_path, mug_animation_path);
  --     else
  --       Net.message_player(player_id, "OH NO! I HOPE YOUR DAY GETS BETTER.", mug_texture_path, mug_animation_path);
  --     end

  --     Net.unlock_player_input(player_id)
  -- end)
end)

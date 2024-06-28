-- local Direction = require("scripts/libs/direction")

-- local area_id = "default"

-- local all_objects = Net.list_objects(area_id)
-- print("All objects:")
-- print(all_objects)

-- for i, object_id in ipairs(all_objects) do
--     local object = Net.get_object_by_id(object_id)
--     print("Object:")
--     print(object)
--     if object.name ~= "Underfloor Bot" then
--         return
--     end
--     local bot_pos = Net.get_position(object.id)
--     local underfloor_bot_id = Net.create_bot({
--       name = "Underfloor Bot",
--       area_id = area_id,
--       texture_path = "/server/assets/community/prog.png",
--       animation_path = "/server/assets/community/prog.animation",
--       x = bot_pos.x,
--       y = bot_pos.y,
--       z = bot_pos.z,
--       solid = true
--     })
--     Net.set_bot_direction(underfloor_bot_id, Direction.DOWN_RIGHT)
-- end
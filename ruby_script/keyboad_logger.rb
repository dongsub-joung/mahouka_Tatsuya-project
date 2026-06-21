class KeyboardObj
  attr_accessor input_way

  def initialize()
    # or Os.io.keyboard
    @input_way= Os.io.Keyboard_data_bus 
  end
end

class KeyboardHandler
  attr_accessor a_key
  
  def initialize(one_keyboad_obj)
    @a_key= one_keyboad_obj
  end
end

# itit
loop
  # new set keyboard object up 
  keyboard_obj= KeyboardObj.new()
  # handle that input
  keyboard_handler= KeyboardHandler.new(keyboard_obj)
  
  
end

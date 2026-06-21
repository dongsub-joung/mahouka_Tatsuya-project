class KeyboardObj
  attr_accessor input_way

  def initialize()
    # or Os.io.keyboard
    @input_way= Os.io.Keyboard_data_bus 
  end
end

class KeyboardHandler
  attr_accessor buffer
  
  def initialize(one_keyboad_obj)
    @buffer= one_keyboad_obj
  end

  def send_buffer()
    begin
      if @buffer.size() >= 30
        # send buffer
        @buffer= []
    rescue -> e 
      @buffer= []
      puts "#{e}: buffer size err"
    end
  end
end

# itit
# new set keyboard object up 
keyboard_obj= KeyboardObj.new()
# handle that input
keyboard_handler= KeyboardHandler.new(one_keyboard_obj)

do
  keyboard_handler.send_buffer()
end

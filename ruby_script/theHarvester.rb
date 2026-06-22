class Data
  def set_data(gethered_data)
    # @TODO migration bot's data -> Data obj
  end
end

class Havester
    attr_accessor nickname
    attr_accessor url
    
    class Bot
      def get_data()
        response= request.get_url(self.nickname, self.url)
        # @TODO handle response as Data format
        return data
    end


    def initialize(_user_nickname)
        @nickname= _user_nickname
    end

    def set_targer_url(url)
      @url= url
    end
    
    def crowing_bot()
      bot= self.Bot.new()
      
      begin
        bot.init
      rescue CrowingErr
        "failed bot init"
      ensure
        return Data.new()
      end

      gethered_data= bot.get_data()
      return Data.set_data(gethered_data)
    end
end

nickname= 'dongsub'
url= "https://github.com"

h= Havester.new(nickname)
puts h.user_nickname

h.set_targer_url(url)
puts h.url

data= h.crowing_bot()
puts data

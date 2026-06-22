class Havester
    attr_accessor nickname
    attr_accessor url

    def initialize(_user_nickname)
        @nickname= _user_nickname
    end

    def set_targer_url(url)
      @url= url
    end

end

nickname= 'dongsub'
url= "https://github.com"

h= Havester.new(nickname)
puts h.user_nickname

h.set_targer_url(url)
puts h.url

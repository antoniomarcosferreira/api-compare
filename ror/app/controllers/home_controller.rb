class HomeController < ApplicationController
  def index
    render json: "Welcome Ruby on Rails"
  end

  def sleep100
    sleep(0.1) # sleep 100 milesecounds
    render json: "RoR 5*5 = #{5*5}"
  end

  def inc
    balance = 0

    threads = []
 
    50.times do |i|
     threads << Thread.new do     
        sleep(0.1)
        balance += 1
        puts "Incremented: #{balance}"
     end
    end
    threads.map(&:join) 
 
    render json: "RoR balance: #{balance}"
  end
end

class Time
  def to_milliseconds
    (self.to_f * 1000.0).to_i
  end
end
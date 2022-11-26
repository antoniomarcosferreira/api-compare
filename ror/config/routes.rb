Rails.application.routes.draw do 
 
  get 'inc', to:'home#inc' 
  get 'sleep100', to: 'home#sleep100' 
  root "home#index"
end

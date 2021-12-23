function uecs --wraps='modsync usr/sys; modsync usr/cmp; modsync usr' --description 'alias uecs modsync usr/sys; modsync usr/cmp; modsync usr'
  modsync usr/sys; modsync usr/cmp; modsync usr $argv; 
end

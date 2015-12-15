#!/usr/bin/ruby


require 'pp'


burn_list = {}
bin_size = 23
bins = []
set_counter = 0

File.open("/home/thenozzle/burnlist",'r').each_line  do |line|
  burn_list[line.split("\t")[1]] = [line.split("\t")[0][0..-2].to_f,false]
end

sorted_burn_list = burn_list.sort_by {|key,value| value[0]}.reverse

sorted_burn_list.each  do |item|
  if item[1][0] < bin_size and !item[1][1]
    bins[set_counter] = {item[0] => item[1][0]}
    #mark as used
    item[1][1] = true
    #now find the next fitting item
    sorted_burn_list.each do |filler_item|
      next if filler_item[1][1]
      if filler_item[1][0] + bins[set_counter].values.inject{|sum,x| sum + x.to_f} <= bin_size
        bins[set_counter][filler_item[0]] = filler_item[1][0]
        #mark this one as well
        filler_item[1][1] = true
      end
    end
  else
    next
  end
  set_counter += 1
end

set_counter = 200
size_total = 0
item_total = 0

bins.each do |item|
  #make the isos...
  print "mkisofs -udf -iso-level 3 -o /home/thenozzle/isos/bin#{set_counter}.iso --graft-points "
  #print "cp -rf "
  item.keys.each {|key| print "'#{key.strip.split('/').last}=#{key.strip}'" + " "}
  item_total += item.keys.size
  size_total += item.values.inject{|sum,x| sum + x.to_f}
  set_counter = set_counter + 1
  #print " /media/flash"
  puts
end

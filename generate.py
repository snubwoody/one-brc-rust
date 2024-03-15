import random
import timeit

def get_stations(file_name:str) -> list:
	stations = []
	file = open(file_name,'r')
	for line in file:
		new_line = line.replace(";","")
		stations.append(new_line.replace("\n",""))
	file.close()
	return stations
	
def generate_lines(count:int,file_name:str,stations:list):
	with open(file_name,'w',buffering=4096) as file:
		for num in range(count):
			random_number = random.uniform(-20.0,50.0)
			file.writelines(f"{random.choice(stations)};{round(random_number,1)}\n")


stations = get_stations('./weather-stations.txt')
generate_lines(1000000000,'./weather-measurements.txt',stations)





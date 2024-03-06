import random

def get_stations(file_name:str) -> list:
	stations = []
	file = open(file_name,'r')
	for line in file:
		new_line = line.replace(";","")
		stations.append(new_line.replace("\n",""))
	file.close()
	return stations
	
def generate_lines(count:int,file_name:str,stations:list):
	file = open(file_name,'w')
	for num in range(count):
		random_number = random.randrange(-20,70)
		file.writelines(f"{random.choice(stations)};{random_number}\n")
	file.close()


stations = get_stations('./weather-stations.txt')
generate_lines(10000,'./measurements.txt',stations)

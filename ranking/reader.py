import json
import requests
import networkx as nx
from copy import deepcopy

def string_format(s):
	return s.strip().lower()

def logistic(x):
	return 1/(1+10**(x/500))

class Player:
	def __init__(self, name):
		self.name = string_format(name)

	def is_equal(self, name):
		return self.name == string_format(name)

	def __eq__(self, other):
		return self.name == other.name

	def __str__(self):
		return self.name[0].upper() + self.name[1:]

	def __repr__(self):
		return str(self)


class Team:
	def __init__(self, p1, p2, id_):
		self.players = (p1,p2)
		self.ranked = False
		self.id = id_

	def __eq__(self, other):
		return (self.players[0]==other.players[0] and self.players[1]==other.players[1]) or (self.players[0]==other.players[1] and self.players[1]==other.players[0])

	def __str__(self):
		return f"({self.players[0]} and {self.players[1]})"

	def __repr__(self):
		return str(self)

	def __hash__(self):
		return hash(self.id)

class Match:
	def __init__(self, t1, t2, p1, p2, date):
		self.team1 = t1
		self.team2 = t2
		self.point1 = p1
		self.point2 = p2
		self.date = date

	def score1(self):
		return self.point1/(self.point1+self.point2)

	def score2(self):
		return self.point2/(self.point1+self.point2)

	def get_data(self):
		return [str(self.team1), str(self.team2), str(self.point1), str(self.point2), str(self.date)]

	def __str__(self):
		return f"{self.team1} vs {self.team2} : {self.point1}/{self.point2}"

	def __repr__(self):
		return str(self)


class Data:
	def __init__(self):
		self.players = []
		self.elo = {}
		self.games = {}
		self.won = {}
		self.lost = {}
		self.teams = []
		self.matches = []
		self.graph = nx.DiGraph()

	def add_player(self, name):
		for p in self.players:
			if p.is_equal(name):
				return p
		self.players.append(Player(name))
		self.elo[self.players[-1].name] = 1000
		self.games[self.players[-1].name] = 0
		self.won[self.players[-1].name] = 0
		self.lost[self.players[-1].name] = 0
		return self.players[-1]

	def add_team(self, name1, name2):
		p1 = self.add_player(name1)
		p2 = self.add_player(name2)
		tmp = Team(p1,p2, -1)
		for t in self.teams:
			if t == tmp:
				return t
		tmp.id = len(self.teams)
		self.teams.append(tmp)
		return self.teams[-1]

	def update_elo(self, t1, t2, point1, point2):
		R11 = self.elo[t1.players[0].name]
		R12 = self.elo[t1.players[1].name]
		R21 = self.elo[t2.players[0].name]
		R22 = self.elo[t2.players[1].name]

		R1 = (R11 + R12)/2
		R2 = (R21 + R22)/2

		E1 = logistic(R2-R1)
		E2 = logistic(R1-R2)

		total = point1 + point2
		S1 = point1/total
		S2 = point2/total

		self.elo[t1.players[0].name] = R11 + 100 * (S1 - E1)
		self.elo[t1.players[1].name] = R12 + 100 * (S1 - E1)
		self.elo[t2.players[0].name] = R21 + 100 * (S2 - E2)
		self.elo[t2.players[1].name] = R22 + 100 * (S2 - E2)

		self.games[t1.players[0].name] += 1
		self.games[t1.players[1].name] += 1
		self.games[t2.players[0].name] += 1
		self.games[t2.players[1].name] += 1
		
		if point1 > point2:
			self.won[t1.players[0].name] += 1
			self.won[t1.players[1].name] += 1
			self.lost[t2.players[0].name] += 1
			self.lost[t2.players[1].name] += 1
		else:
			self.lost[t1.players[0].name] += 1
			self.lost[t1.players[1].name] += 1
			self.won[t2.players[0].name] += 1
			self.won[t2.players[1].name] += 1

	def add_match(self, name11, name12, name21, name22, point1, point2, date):
		t1 = self.add_team(name11,name12)
		t2 = self.add_team(name21,name22)
		self.matches.append(Match(t1,t2,point1,point2,date))
		self.update_elo(t1,t2,point1,point2)
		self.add_edge(t1,t2,point1,point2)
		self.add_edge(t2,t1,point2,point1)

	def add_edge(self, t1, t2, point1, point2):
		self.graph.add_edge(t1,t2)
		if "points" not in self.graph[t1][t2]:
			self.graph[t1][t2]["points"] = 0
			self.graph[t1][t2]["total"] = 0
		self.graph[t1][t2]["points"] += point1
		self.graph[t1][t2]["total"] += point1+point2

	def score_all(self):
		for e in list(self.graph.edges()):
			self.graph[e[0]][e[1]]["score"] = self.graph[e[0]][e[1]]["points"]/self.graph[e[0]][e[1]]["total"]

	def get_indiv_ranking(self):
		l = deepcopy(self.players)
		l.sort(key=lambda x:-self.elo[x.name])
		ret = []
		for rank in range(len(l)):
			score = int(self.elo[l[rank].name])
			won = self.won[l[rank].name]
			lost = self.lost[l[rank].name]
			games = self.games[l[rank].name]
			ret.append([str(l[rank]), str(score), str(rank+1), str(games), str(won), str(lost), f"{int(100*won/games)}%"])
		return ret

	def __str__(self):
		return "Players: {}\nTeams: {}\nMatches:\n{}".format(
			self.elo,
			" ".join(list(map(str,self.teams))),
			"\n".join(list(map(str,self.matches))))

	def __repr__(self):
		return str(self)



def read():
	x = requests.get('https://eu.kobotoolbox.org/api/v2/assets/aBX2f3MzWmsrEegeWNqbB8/data/?format=json')
	x = x.json()
	ret = Data()
	players = []
	teams = []
	matches = []
	for r in x["results"]:
		ret.add_match(r["Team_1_Player_1"], r["Team_1_Player_2"], r["Team_2_Player_1"], r["Team_2_Player_2"], int(r["Team_1_Points"]), int(r["Team_2_Points"]), r["today"])
	ret.score_all()
	return ret
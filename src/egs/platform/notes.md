# Get Player Profile

	https://store.epicgames.com/graphql?operationName=playerProfile&variables=%7B%22epicAccountId%22:%22d6489fab704147beb2689b994fab4935%22%7D&extensions=%7B%22persistedQuery%22:%7B%22version%22:1,%22sha256Hash%22:%22ff954147a23d38a0e5b050962d442099487da001a0ab4b10ccbec8ac49755b3c%22%7D%7D
	
	{"epicAccountId":"d6489fab704147beb2689b994fab4935"}
	{"persistedQuery":{"version":1,"sha256Hash":"ff954147a23d38a0e5b050962d442099487da001a0ab4b10ccbec8ac49755b3c"}}

## Necessary Parameters
- epicAccountId
- sha256Hash

## Response Format

	{
		"data":
		{
			"PlayerProfile":
			{
				"playerProfile":
				{
					"epicAccountId": "d6489fab704147beb2689b994fab4935",
					"displayName": "NemesisX00",
					"avatar":
					{
						"small": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=64",
						"medium": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=128",
						"large": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=512"
					}
				}
			}
		}
	}

&nbsp;

# Get Player Profile Private

Get the achievements summary for the profile.

Because we're avoiding authentication as much as possible, in order to minimize the security risk this app presents, the user's profile must have its privacy level set to public

	https://store.epicgames.com/graphql?operationName=playerProfilePrivate&variables=%7B%22epicAccountId%22:%22d6489fab704147beb2689b994fab4935%22,%22locale%22:%22en-US%22,%22page%22:1,%22accountId%22:%22d6489fab704147beb2689b994fab4935%22%7D&extensions=%7B%22persistedQuery%22:%7B%22version%22:1,%22sha256Hash%22:%2247d0391fa5ec42d829e4a03f399cb586a29cf3cebd940cc4747aed0192c61114%22%7D%7D
	
	{"epicAccountId":"d6489fab704147beb2689b994fab4935","locale":"en-US","page":1,"accountId":"d6489fab704147beb2689b994fab4935"}
	{"persistedQuery":{"version":1,"sha256Hash":"47d0391fa5ec42d829e4a03f399cb586a29cf3cebd940cc4747aed0192c61114"}}

## Necessary Parameters
- operationName
- epicAccountId
- locale
- page
- accountId
- sha256Hash

## Response Format

	{
		"data": {
			"PlayerProfile": {
				"playerProfile": {
					"privacy": null,
					"relationship": "NO_RELATIONSHIP",
					"achievementsSummaries": {
						"__typename": "PlayerAchievementResponseSuccess",
						"status": 200,
						"data": [
							{
								"totalUnlocked": 7,
								"totalXP": 50,
								"sandboxId": "3eb832eb6d9f4496818a0b0d667420c0",
								"baseOfferForSandbox": {
									"keyImages": [
									{
										"url": "https://cdn1.epicgames.com/spt-assets/98cdc7484b264021ad8a3a6e1e721989/dnf-duel-1alsy.jpg",
										"type": "OfferImageWide",
										"alt": "DNF Duel Offer"
									},
									{
										"url": "https://cdn1.epicgames.com/spt-assets/98cdc7484b264021ad8a3a6e1e721989/dnf-duel-53i3x.jpg",
										"type": "OfferImageTall",
										"alt": "Download DNF Duel Offer"
									},
									{
										"url": "https://cdn1.epicgames.com/spt-assets/98cdc7484b264021ad8a3a6e1e721989/dnf-duel-53i3x.jpg",
										"type": "Thumbnail",
										"alt": "Download DNF Duel Offer"
									}
									]
								},
								"product": {
									"name": "DNF Duel",
									"slug": "dnf-duel-07d055"
								},
								"productAchievements": {
									"totalAchievements": 43,
									"totalProductXP": 1000
								},
								"playerAwards": []
							},
							...
						]
					},
					"friendsSummaries": {
						"__typename": "PlayerFriendResponseSuccess",
						"status": 200,
						"data": {
							"page": 1,
							"nextPage": null,
							"previousPage": null,
							"totalPages": 1,
							"totalItems": 1,
							"content": [
								{
									"epicAccountId": "6b97a1ea7cbf428895d9ac72352d51fa",
									"displayName": "Keesa",
									"avatar": {
									"small": "https://shared-static-prod.epicgames.com/epic-profile-icon/4D1397/K/icon.png?size=64",
									"medium": "https://shared-static-prod.epicgames.com/epic-profile-icon/4D1397/K/icon.png?size=128",
									"large": "https://shared-static-prod.epicgames.com/epic-profile-icon/4D1397/K/icon.png?size=512"
									}
								}
							]
						}
					}
				}
			},
			"Friends": {
				"summary": {
					"outgoing": [],
					"blocklist": []
				}
			}
		}
	}

&nbsp;

# Get Achievements List For Game
	
	https://store.epicgames.com/graphql?operationName=Achievement&variables=%7B%22sandboxId%22:%223eb832eb6d9f4496818a0b0d667420c0%22,%22locale%22:%22en-US%22%7D&extensions=%7B%22persistedQuery%22:%7B%22version%22:1,%22sha256Hash%22:%229284d2fe200e351d1496feda728db23bb52bfd379b236fc3ceca746c1f1b33f2%22%7D%7D
	
	{"sandboxId":"3eb832eb6d9f4496818a0b0d667420c0","locale":"en-US"}
	
	{"persistedQuery":{"version":1,"sha256Hash":"9284d2fe200e351d1496feda728db23bb52bfd379b236fc3ceca746c1f1b33f2"}}
	
	https://store.epicgames.com/graphql?operationName=Achievement&variables={"sandboxId":"3eb832eb6d9f4496818a0b0d667420c0","locale":"en-US"}&extensions={"persistedQuery":{"version":1,"sha256Hash":"9284d2fe200e351d1496feda728db23bb52bfd379b236fc3ceca746c1f1b33f2"}}

## Necessary Parameters
- operationName
- sandboxId
- locale
- sha256Hash

## Response Format

	{
		"data": {
			"Achievement": {
				"productAchievementsRecordBySandbox": {
					"productId": "98cdc7484b264021ad8a3a6e1e721989",
					"sandboxId": "3eb832eb6d9f4496818a0b0d667420c0",
					"totalAchievements": 43,
					"totalProductXP": 1000,
					"achievementSets": [
					{
						"achievementSetId": "dge1ook",
						"isBase": true,
						"totalAchievements": 43,
						"totalXP": 1000
					}
					],
					"platinumRarity": {
					"percent": 0.1
					},
					"achievements": [
						{
							"achievement": {
								"sandboxId": "3eb832eb6d9f4496818a0b0d667420c0",
								"deploymentId": "d78a27208ef944ff9a592be5bf975f02",
								"name": "ACHIEVEMENT_001",
								"hidden": false,
								"isBase": true,
								"achievementSetId": "dge1ook",
								"unlockedDisplayName": "Gold Merchant",
								"lockedDisplayName": "Gold Merchant",
								"unlockedDescription": "Spend 50,000Gold",
								"lockedDescription": "Spend 50,000Gold",
								"unlockedIconId": "TROP001.jpg",
								"lockedIconId": "TROP001_g.jpg",
								"XP": 10,
								"flavorText": "",
								"unlockedIconLink": "https://shared-static-prod.epicgames.com/epic-achievements/98cdc7484b264021ad8a3a6e1e721989/3eb832eb6d9f4496818a0b0d667420c0/icons/4a995b6ff20b57edfbc9c247c969b88e",
								"lockedIconLink": "https://shared-static-prod.epicgames.com/epic-achievements/98cdc7484b264021ad8a3a6e1e721989/3eb832eb6d9f4496818a0b0d667420c0/icons/3636ea9de064e17315ec3b84e781c3d3",
								"tier": {
									"name": "bronze",
									"hexColor": "#CA512B",
									"min": 0,
									"max": 45
								},
								"rarity": {
									"percent": 0.1
								}
							}
						},
						...
					]
				}
			}
		}
	}

&nbsp;

# Get Achievement Progress For User And Game
	
	https://store.epicgames.com/graphql?operationName=playerProfileAchievementsByProductId&variables=%7B%22epicAccountId%22:%22d6489fab704147beb2689b994fab4935%22,%22productId%22:%2298cdc7484b264021ad8a3a6e1e721989%22%7D&extensions=%7B%22persistedQuery%22:%7B%22version%22:1,%22sha256Hash%22:%2270ff714976f88a85aafa3cb5abb9909d52e12a3ff585d7b49550d2493a528fb0%22%7D%7D
	
	{"epicAccountId":"d6489fab704147beb2689b994fab4935","productId":"98cdc7484b264021ad8a3a6e1e721989"}
	
	{"persistedQuery":{"version":1,"sha256Hash":"70ff714976f88a85aafa3cb5abb9909d52e12a3ff585d7b49550d2493a528fb0"}}
	
	https://store.epicgames.com/graphql?operationName=playerProfileAchievementsByProductId&variables={"epicAccountId":"d6489fab704147beb2689b994fab4935","productId":"98cdc7484b264021ad8a3a6e1e721989"}&extensions={"persistedQuery":{"version":1,"sha256Hash":"70ff714976f88a85aafa3cb5abb9909d52e12a3ff585d7b49550d2493a528fb0"}}

## Necessary Parameters
- operationName
- epicAccountId
- productId
- sha256Hash

## Response Format

	{
		"data": {
			"PlayerProfile": {
				"playerProfile": {
					"epicAccountId": "d6489fab704147beb2689b994fab4935",
					"displayName": "NemesisX00",
					"relationship": "NO_RELATIONSHIP",
					"avatar": {
					"small": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=64",
					"medium": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=128",
					"large": "https://shared-static-prod.epicgames.com/epic-profile-icon/FF6D32/N/icon.png?size=512"
					},
					"productAchievements": {
						"__typename": "PlayerProductAchievementsResponseSuccess",
						"data": {
							"epicAccountId": "d6489fab704147beb2689b994fab4935",
							"sandboxId": "3eb832eb6d9f4496818a0b0d667420c0",
							"totalXP": 50,
							"totalUnlocked": 7,
							"achievementSets": [
								{
									"achievementSetId": "dge1ook",
									"isBase": true,
									"totalUnlocked": 7,
									"totalXP": 50
								}
							],
							"playerAwards": [],
							"playerAchievements": [
								{
									"playerAchievement": {
									"achievementName": "ACHIEVEMENT_005",
									"epicAccountId": "d6489fab704147beb2689b994fab4935",
									"progress": 1,
									"sandboxId": "3eb832eb6d9f4496818a0b0d667420c0",
									"unlocked": true,
									"unlockDate": "2024-10-09T07:04:55.023Z",
									"XP": 10,
									"achievementSetId": "dge1ook",
									"isBase": true
									}
								},
								...
							]
						}
					}
				}
			}
		}
	}

&nbsp;

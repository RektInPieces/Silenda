interface Card {
  name: string;
  image: string;
  onTurnAction: Action;
}

interface Action {}

export default Card;

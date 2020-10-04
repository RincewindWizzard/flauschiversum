import unittest
import requests


"""
* Url to ermelines feed as json:
*
*       https://www.instagram.com/ermelinewollknoll/?__a=1&max_id=<endcursor>
*
* For pagination use the endcursor to get to the next page:
*
*       jq .graphql.user.edge_owner_to_timeline_media.page_info.end_cursor
*
* This is the jq path to the media description text:
*
*       jq .graphql.user.edge_owner_to_timeline_media.edges[].node.edge_media_to_caption.edges[].node.text
*
* This is the jq path to the media url:
*
*       jq .graphql.user.edge_owner_to_timeline_media.edges[].node.display_url
*
* Download all images:
*       for i in $(cat ermeline_insta.json  | jq .graphql.user.edge_owner_to_timeline_media.edges[].node.display_url | sed 's/"//g'); do dst=$(echo $i | sed 's/^.*\///g' | sed 's/jpg.*$/jpg/g'); wget "$i" -O download/$dst;  done;
*

"""

api_endpoint = 'https://www.instagram.com/ermelinewollknoll/?__a=1'

def get_media_list(feed):
    for node in feed['graphql']['user']['edge_owner_to_timeline_media']['edges']:
        display_url = node['node']['display_url']
        caption = [x['node']['text'] for x in node['node']['edge_media_to_caption']['edges']][0]

        yield (display_url, caption)

def get_endcursor(feed):
    return feed['graphql']['user']['edge_owner_to_timeline_media']['page_info']['end_cursor']

def request_user_feed(endcursor=None):
    url = '{}&max_id={}'.format(api_endpoint, endcursor) if endcursor else api_endpoint
    print(url)
    feed = requests.get(url).json()

    next_endcursor = get_endcursor(feed)
    print(next_endcursor)

    media = list(get_media_list(feed))
    print(repr(media[0][1]))

    if len(media) > 0 and next_endcursor:
        request_user_feed(next_endcursor)

class InstagramTest(unittest.TestCase):
    def test_list_posts(self):
        request_user_feed()

        #for display_url, caption in get_media_list(feed):
        #    print(repr(caption))


if __name__ == '__main__':
    unittest.main()

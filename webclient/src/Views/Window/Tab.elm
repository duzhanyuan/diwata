module Views.Window.Tab exposing (listView, Model, init, update, Msg)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, property)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row
import Window as BrowserWindow
import Task exposing (Task)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Html.Events exposing (on)
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode 
import Util exposing ((=>))


type alias Model =
    { tab : Tab
    , browserSize: BrowserWindow.Size
    , listRowScroll: Scroll
    }

type alias Scroll =
    { top: Int
    , left: Int
    }

init: Tab -> Task PageLoadError Model
init tab =
    let 
        browserSize = BrowserWindow.size
    in
        Task.map (\size ->
            { tab = tab
            , browserSize = size
            , listRowScroll = Scroll 0 0
            }
        ) browserSize

listView: Model -> Rows -> Html Msg
listView model rows =
    let 
        tab = model.tab
        columnNames = Tab.columnNames tab
        fields = tab.fields
        recordList = Record.rowsToRecordList rows
        recordIdList = 
            List.map (\record -> Tab.recordId record tab) recordList

    in
    div [class "tab-list-view"] 
        [ div [class "frozen-head-columns"]
            [ viewFrozenHead model
            , viewColumns model fields
            ]
        , div [class "row-shadow-list-rows"]
            [ viewRowShadow model
            , listViewRows tab recordIdList recordList
            ]
        ]


viewRowShadow: Model -> Html msg
viewRowShadow model =
    let 
        scrollTop = model.listRowScroll.top
    in
    div [ class "row-shadow"
        , property "scrollTop" (Encode.int scrollTop)
        ]
        [ div [class "row-shadow-content"]
            (List.map
                ( \ i ->
                    div []
                        [text ("Row shadow "++toString i)]
                ) (List.range  1 40)
            )
        ]

viewFrozenHead: Model -> Html msg
viewFrozenHead model =
    div [ class "frozen-head"
        ]
        [ text "frozen head"]

viewColumns: Model -> List Field -> Html msg
viewColumns model fields =
    let 
        scrollLeft = model.listRowScroll.left
    in
    div [ class "tab-columns"
        , property "scrollLeft" (Encode.int scrollLeft)
        ]
        [ div [class "tab-columns-content"]
            (List.map viewColumn fields)
        ]

viewColumn: Field -> Html msg
viewColumn field =
    div [class "tab-column"]
        [text (Field.columnName field)]

listViewRows: Tab -> List RecordId -> List Record -> Html Msg
listViewRows tab recordIdList recordList =
    div [class "list-view-rows", onScroll] 
        (
        if List.length recordList > 0 then
            (List.map2 
                (\ recordId record ->
                    Row.view recordId record tab
                )
                recordIdList recordList
             )
        else
            [div [class "empty-list-view-rows"]
                [text "Empty list view rows"]
            ]
        )

onScroll: Attribute Msg
onScroll =
    on "scroll" (Decode.map ListRowScrolled scrollDecoder)

scrollDecoder: Decoder Scroll
scrollDecoder =
    Decode.map2 Scroll
        (Decode.at ["target", "scrollTop"] Decode.int)
        (Decode.at ["target", "scrollLeft"] Decode.int)


type Msg
    = WindowResized BrowserWindow.Size
    | ListRowScrolled Scroll

update: Msg -> Model ->  (Model, Cmd Msg)
update msg model =
    let
        _ = Debug.log "Update tab msg " msg
    in
    case msg of
        WindowResized size ->
            { model | browserSize = size } => Cmd.none

        ListRowScrolled scroll ->
            { model | listRowScroll = scroll } => Cmd.none



subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\ size -> WindowResized size)
        ] 
